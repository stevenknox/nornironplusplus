import NextHead from 'next/head'
import React from 'react'

type MetaTag = {
  name: string
  content: string
}

const metaTag = (name: string, content: string) => ({ name, content })

const title = 'NornIron++'
const description = 'The programming language for our wee country.'
const imageUrl = '/image.png'
const author = '@stevenknox101'

const metaTags: MetaTag[] = [
  metaTag('description', description),
  metaTag('og:title', title),
  metaTag('og:description', description),
  metaTag('og:image', imageUrl),
  metaTag('twitter:card', 'summary'),
  metaTag('twitter:site', title),
  metaTag('twitter:title', title),
  metaTag('twitter:creator', author),
  metaTag('twitter:description', description),
  metaTag('twitter:image', imageUrl)
]

const Head = () => (
  <NextHead>
    <title>{title}</title>
    {metaTags.map(({ name, content }) => (
      <meta key={name + content} name={name} content={content} />
    ))}
  </NextHead>
)

export default Head
