import RemoteMdxPage from './mdxremote/page'

export default function Home() {
  return (
    <main className="flex min-h-screen flex-col">
      <h1>Reading MDX from remote git repo</h1>
      <hr/>
      <RemoteMdxPage></RemoteMdxPage>
    </main>
  )
}
