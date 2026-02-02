import { openUrl } from "@tauri-apps/plugin-opener";

export function About() {
  const handleLinkClick = async (e: React.MouseEvent) => {
    e.preventDefault();
    console.log("Attempting to open URL: https://github.com/ibraved/mecha-parser/issues");
    try {
      await openUrl("https://github.com/ibraved/mecha-parser/issues");
      console.log("URL opened successfully");
    } catch (err) {
      console.error("Caught error in openUrl:", err);
      alert(`Failed to open link: ${err}`);
    }
  };

  return (
    <div className="space-y-3">
      <h2 className="text-2xl font-bold mb-2">Mecha Parser</h2>
      <div className="text-sm text-muted-foreground">
        Mecha Parser is a desktop roster monitor that reads MechaBREAK logs locally and renders a
        live view of teams and readiness.
      </div>
      <div className="rounded-lg border border-border bg-background/20 p-3 text-xs text-muted-foreground">
        If there are any issues, add them here:{" "}
        <a
          href="https://github.com/ibraved/mecha-parser/issues"
          onClick={handleLinkClick}
          className="text-primary hover:underline cursor-pointer"
        >
          https://github.com/ibraved/mecha-parser/issues
        </a>
      </div>
    </div>
  );
}


