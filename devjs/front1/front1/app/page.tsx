import RemoteMdxPage from './mdxremote/page'

export default function Home() {
  return (
    <main className="flex min-h-screen flex-col">
      Server: {new Date().toISOString()}
      
      <hr/>

      <RemoteMdxPage></RemoteMdxPage>
      <hr/> 
      Server: {new Date().toISOString()}
    </main>
  )
}
