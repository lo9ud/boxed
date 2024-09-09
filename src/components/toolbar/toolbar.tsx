import "./toolbar.css";

export default function Toolbar() {
  return (
    <div className="toolbar">
      <ToolbarSection name={"File"}>
        <ToolbarButton icon="New" />
        <ToolbarButton icon="Open" />
        <ToolbarButton icon="Save" />
      </ToolbarSection>
      <ToolbarSpacer />
      <ToolbarSection name={"Column"}>
        <ToolbarButton icon="New" />
        <ToolbarButton icon="Edit" />
        <ToolbarButton icon="Remove" />
      </ToolbarSection>
      <ToolbarSpacer />
      <ToolbarSection name={"Formula"}>
        <ToolbarButton icon="New" />
        <ToolbarButton icon="Edit" />
        <ToolbarButton icon="Remove" />
      </ToolbarSection>
    </div>
  );
}

function ToolbarSpacer() {
  return <div className="toolbar-spacer" />;
}

function ToolbarSection({
  name,
  children,
}: {
  name?: String;
  children: React.ReactNode;
}) {
  return (
    <div className="toolbar-section">
      <div className="toolbar-section-inner">{children}</div>
      <div className="toolbar-section-title">{name}</div>
    </div>
  );
}

function ToolbarButton({ icon }: { icon: string }) {
  return <button className="toolbar-button">{icon}</button>;
}
