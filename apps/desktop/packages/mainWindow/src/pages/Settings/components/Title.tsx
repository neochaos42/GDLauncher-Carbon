import { JSX, Show, children } from "solid-js"

interface Props {
  children: JSX.Element
  description?: JSX.Element
  class?: string
}

function Title(props: Props) {
  const c = children(() => props.children)

  return (
    <div class={props.class || undefined}>
      <h4 class="text-lg font-medium text-lightSlate-100">{c()}</h4>
      <Show when={props.description}>
        <p class="text-lightSlate-700 max-w-200 pr-4">{props.description}</p>
      </Show>
    </div>
  )
}

export default Title
