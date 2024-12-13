import { JSX, children } from "solid-js"

interface Props {
  children: JSX.Element
}

function PageTitle(props: Props) {
  const c = children(() => props.children)

  return <h3 class="mt-0">{c()}</h3>
}

export default PageTitle
