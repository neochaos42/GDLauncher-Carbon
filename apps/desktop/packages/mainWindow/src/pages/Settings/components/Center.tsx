import { JSX, children } from "solid-js"

interface Props {
  children: JSX.Element
  class?: string
}

function Center(props: Props) {
  const c = children(() => props.children)

  return (
    <div class={"flex gap-4 justify-center items-center w-full " + props.class}>
      {c()}
    </div>
  )
}

export default Center
