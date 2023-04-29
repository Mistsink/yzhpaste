export const useScrollToSelectedItem = (
  scrollContainer: HTMLElement,
  selectedItem: HTMLElement,
  eta: number = 1
) => {
  const containerRect = scrollContainer.getBoundingClientRect()
  const itemRect = selectedItem.getBoundingClientRect()

  // 动态获取 1rem 的大小
  const delta = parseFloat(getComputedStyle(document.documentElement).fontSize) * eta

  let start: number | null
  const startScrollLeft = scrollContainer.scrollLeft
  let scrollTo = 0

  if (itemRect.left < containerRect.left + delta) {
    scrollTo = scrollContainer.scrollLeft + (itemRect.left - containerRect.left) - delta
  } else if (itemRect.right > containerRect.right - delta) {
    scrollTo = scrollContainer.scrollLeft + (itemRect.right - containerRect.right) + delta
  } else {
    return // 不需要滚动
  }

  const duration = 300 // 动画持续时间（毫秒）
  const bezier = (t: number) => t * (2 - t) // 贝塞尔曲线函数，可以根据需要调整

  const step = (timestamp: number) => {
    if (!start) start = timestamp
    const progress = timestamp - start
    const t = progress / duration

    scrollContainer.scrollLeft = startScrollLeft + bezier(t) * (scrollTo - startScrollLeft)

    if (progress < duration) {
      window.requestAnimationFrame(step)
    } else {
      scrollContainer.scrollLeft = scrollTo // 确保滚动到正确的位置
    }
  }
  window.requestAnimationFrame(step)
}
