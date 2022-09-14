export const scheduler = {
  debug: false,
  runTaskInstantly: false,
  tasks: [] as VoidFunction[],
  run() {
    if (!this.tasks.length) {
      if (this.debug) {
        console.debug(`scheduler: task list is empty`)
      }
      return
    }

    const tasks = this.tasks
    this.tasks = []

    tasks.forEach(fn => fn())

    if (this.debug) {
      console.debug(`Ran ${tasks.length} task(s)`)
    }
  },

  schedule(task: VoidFunction) {
    if (this.runTaskInstantly) {
      return task()
    }

    this.tasks.push(task)
  },
}

export type LazyInitFn<T> = () => T
export const lazy = <T>(initFn: LazyInitFn<T>): T => {
  const result = {} as T

  scheduler.schedule(() => {
    const value = initFn()

    Object.assign(result, value)
  })

  return result
}
