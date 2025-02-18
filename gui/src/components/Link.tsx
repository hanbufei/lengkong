import type { Component } from 'solid-js'

interface LinkProps {
  href: string
  children: string
}

const Link: Component<LinkProps> = props => {
  return (
    <a href={props.href} target="_blank" class="text-orange-500">
      {props.children}
    </a>
  )
}

export default Link
