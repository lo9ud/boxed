import StatusLine from "@/components/statusline/statusline";
import Toolbar from "@/components/toolbar/toolbar";
import GridDisplay from "@/components/grid/grid";
export default function Home() {
  return (
    <main>
      <Toolbar />
      <GridDisplay />
      <StatusLine />
    </main>
  );
}
