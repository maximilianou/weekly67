import { MDXRemote, MDXRemoteSerializeResult } from 'next-mdx-remote/rsc'
import { serialize } from 'next-mdx-remote/serialize'

interface Props {
  mdxSource: MDXRemoteSerializeResult
}

export default async function RemoteMdxPage() {
  // MDX text - can be from a local file, database, CMS, fetch, anywhere...
  const res = await fetch('https://raw.githubusercontent.com/maximilianou/weekly67/main/devjs/README.md');
  const markdown = await res.text();
  return <MDXRemote source={markdown} />
}