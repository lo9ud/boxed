import "./grid.css";

export default function GridDisplay() {
  let columns = [];
  for (let i = 0; i < 50; i++) {
    let column = [];
    for (let j = 0; j < 50; j++) {
      column.push(
        <div key={j} className="cell">
          {"cell " + i + ":" + j}
        </div>
      );
    }
    columns.push(<Column key={i}>{column}</Column>);
  }
  return (
    <div className="grid">
      <Grid>{columns}</Grid>
      <Workspace></Workspace>
    </div>
  );
}

function Grid({ children }: { children?: React.ReactNode }) {
  return (
    <div className="grid-display">
      <div className="grid-inner">{children}</div>
    </div>
  );
}

function Column({ children }: { children?: React.ReactNode }) {
  return (
    <div className="column">
      <ColumnHeader />
      {children}
      <ColumnFooter />
    </div>
  );
}

function ColumnHeader() {
  return <div className="column-header">COLUMN HEADER</div>;
}

function ColumnFooter() {
  return <div className="column-footer" />;
}

function Workspace({ children }: { children?: React.ReactNode[] }) {
  return (
    <div className="workspace">
      <Globals />
      <DetailView />
    </div>
  );
}

function Globals() {
  return <div className="globals">GLOBALS</div>;
}

function DetailView() {
  return (
    <div className="detail-view">
      DETAIL VIEW GOES HERE <br />
      yadda yadda yadda <br />
      column info: <br />
      cell info: <br />
      type info: <br />
      etc.
    </div>
  );
}
