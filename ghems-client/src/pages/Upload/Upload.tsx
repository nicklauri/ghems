import { useCallback } from "react"
import { ChangeEvent, useRef, useState } from "react"
import { Button, PrimaryButton, SecondaryButton } from "../../components/Button/Button"
import { TextField } from "../../components/TextField/TextField"
import { useBoolean } from "../../hooks/useBoolean"
import { CommonUtils } from "../../utils/common"
import "./Upload.scss"

type FileState = {
  isRenaming: boolean,
  file: File,
}

type FileNameCache = {
  [idx: number]: string,
}

export const Upload = () => {
  const ref = useRef<HTMLInputElement>(null)
  const [files, setFiles] = useState<FileState[]>([])
  const [fileNameCache, setFileNameCache] = useState<FileNameCache>({})
  const [previewFileState, setPreviewFileState] = useState<FileState>()
  const [previewContent, setPreviewContent] = useState("")

  console.debug(`rerender`)

  const onClickBrowseFiles = () => {
    ref.current?.click()
  }

  const onChangeFiles: React.ChangeEventHandler<HTMLInputElement> = (ev) => {
    if (!ev.target.files) {
      // setFiles([])
      return
    }

    const newFileList = Array.from(ev.target.files).map(fileStateFromFile)
    const oldFileList = files.filter(fs => newFileList.every(item => item.file.name !== fs.file.name))
    const fileList = CommonUtils.merge(newFileList, oldFileList)
    // const fileList = CommonUtils.merge(newFileList, files)

    setFiles(fileList)

    console.log(fileList)
  }

  const onSortFiles = () => {
    setFiles([...CommonUtils.sortAtoZ(files, fs => fs.file.name)])
  }

  const renameFile = (idx: number, fs: FileState) => () => {
    fs.isRenaming = true
    setFiles([...files])

    // fileNameCache[idx] = fs.file.name
    // setFileNameCache({ ...fileNameCache })
    // console.debug(`add fileNameCache[${idx}]:`, fileNameCache)
  }

  const saveFileName = (idx: number, fs: FileState) => () => {
    fs.isRenaming = false
    // fs.file = new File([fs.file], fileNameCache[idx])
    setFiles([...files])
    // delete fileNameCache[idx]
    // setFileNameCache({ ...fileNameCache })
    // console.debug(`delete fileNameCache[${idx}]: `, fileNameCache)
  }

  const onChangeFileName = (idx: number, fs: FileState) => (value: string) => {
    if (files[idx] !== fs) {
      console.debug(`somehow, files[${idx}] !== fs:`, fs)
      return;
    }

    // const fileName = event.target.value
    // fileNameCache[idx] = fileName
    // setFileNameCache({ ...fileNameCache })

    const oldFile = fs.file

    console.log(`changed filename from: "${fs.file.name}" => "${value}"\n`, files)
    fs.file = new File([fs.file], value)
    setFiles([...files])

    if (oldFile === previewFileState?.file) {
      setPreviewFileState(fs)
    }
  }

  const previewFileFn = (fs: FileState) => () => {
    setPreviewFileState(fs)

    const read = new FileReader()
    read.readAsText(fs.file)

    read.addEventListener("load", () => {
      setPreviewContent(read.result?.toString() || "")
    }, { once: true })
  }

  const delFileFn = (fs: FileState) => () => {
    const newFileList = files.filter(f => f !== fs)

    if (newFileList.length !== files.length) {
      setFiles(newFileList)
    }

    if (fs.file === previewFileState?.file) {
      setPreviewFileState(undefined)
    }
  }

  return <div className="upload-page">
    <input
      ref={ref}
      type="file"
      multiple
      className="display-none"
      onChange={onChangeFiles}
    />

    <div className="container">
      <div className="row actions">
        <PrimaryButton
          text="Choose file"
          onClick={onClickBrowseFiles}
        />
        <Button
          text="Sort"
          onClick={onSortFiles}
        />
      </div>
      <div className="row">
        <div className="item-list">
          <div className="header">Files</div>
          <div className="list">
            {files.length > 0
              ? files.map((fs, idx) =>
                <div key={idx} className="file">
                  <div className="name">
                    {fs.isRenaming
                      // ? <TextField value={fileNameCache[idx]} onChange={onChangeFileName(idx, fs)} />
                      ? <TextField value={fs.file.name} onChange={onChangeFileName(idx, fs)} />
                      : fs.file.name
                    }
                  </div>
                  <div className="actions">
                    {fs.isRenaming
                      ? <PrimaryButton text="Save" onClick={saveFileName(idx, fs)} />
                      : <Button text="Rename" onClick={renameFile(idx, fs)} />
                    }
                    <PrimaryButton text="Preview" onClick={previewFileFn(fs)} />
                    <SecondaryButton text="Delete" onClick={delFileFn(fs)} />
                  </div>
                </div>)
              : <div className="empty">No file selected.</div>
            }
          </div>
        </div>
      </div>
      {previewFileState &&
        <div className="row">
          <div className="item-list">
            <div className="header">Preview: {previewFileState.file.name}</div>
            <pre className="content">
              {previewContent}
            </pre>
          </div>
        </div>
      }
    </div>
  </div>
}

type EditableTextProps<T> = {
  isEditable: boolean,
  value: string,
  onChange: (data: T, value: string) => void,
  data: T,
}
const EditableText = <T,>(props: EditableTextProps<T>) => {
  return <>
  </>
}

const fileStateFromFile = (file: File): FileState => ({
  isRenaming: false,
  file,
})
