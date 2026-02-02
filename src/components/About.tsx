export function About() {
  return (
    <div className="space-y-3">
      <h2 className="text-2xl font-bold mb-2">Mecha Parser</h2>
      <div className="text-sm text-muted-foreground">
        Mecha Parser is a desktop roster monitor that reads MechaBREAK logs locally and renders a
        live view of teams and readiness.
      </div>
      <div className="rounded-lg border border-border bg-background/20 p-3 text-xs text-muted-foreground">
        This app uses an original, tasteful theme. No official MechaBREAK art/media is bundled by
        default.
      </div>
    </div>
  );
}

