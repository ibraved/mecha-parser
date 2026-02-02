import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { Button } from "@/components/ui/button";
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from "@/components/ui/table";
import { useTracker } from "@/hooks/useTracker";
import { useConfig } from "@/hooks/useConfig";
import type { Player } from "@/types";
import { About } from "@/components/About";
import { getMechaName } from "@/lib/mechaNames";

function Shell({ children }: { children: React.ReactNode }) {
  const tracker = useTracker();
  return (
    <div className="min-h-full">
      <div className="mx-auto flex max-w-6xl flex-col gap-6 px-6 py-6">
        <header className="flex items-center justify-between">
          <div className="flex flex-col">
            <div className="text-sm uppercase tracking-widest text-muted-foreground">
              Mecha Parser
            </div>
            <div className="text-xl font-semibold">Roster Monitor</div>
          </div>
          <div className="flex items-center gap-2">
            <div className="mr-3 text-xs text-muted-foreground">
              {tracker.isTauri ? (
                tracker.status.tracking ? (
                  <span>Tracking</span>
                ) : (
                  <span>Idle</span>
                )
              ) : (
                <span>Web preview (run via Tauri to track)</span>
              )}
            </div>
            <Button
              variant="outline"
              disabled={!tracker.isTauri || tracker.status.tracking}
              onClick={tracker.startTracking}
            >
              Start
            </Button>
            <Button
              variant="secondary"
              disabled={!tracker.isTauri || !tracker.status.tracking}
              onClick={tracker.stopTracking}
            >
              Stop
            </Button>
          </div>
        </header>
        {children}
      </div>
    </div>
  );
}

function TeamTable({ title, players }: { title: string; players: Player[] }) {
  return (
    <div className="rounded-xl border border-border bg-card/60 p-4 backdrop-blur">
      <div className="mb-3 flex items-baseline justify-between">
        <div className="text-sm font-semibold">{title}</div>
        <div className="text-xs text-muted-foreground">{players.length} players</div>
      </div>
      <Table>
        <TableHeader>
          <TableRow>
            <TableHead>Player</TableHead>
            <TableHead>Mecha</TableHead>
            <TableHead>AI</TableHead>
            <TableHead>Ready</TableHead>
          </TableRow>
        </TableHeader>
        <TableBody>
          {players.map((p) => {
            const name = p.displayName ?? `Player ${p.playerId}`;
            const ai = p.isAi;
            const ready = p.ready;
            const nameClass =
              ai === true ? "text-red-300" : ai === undefined ? "text-yellow-200" : "text-foreground";
            return (
              <TableRow key={p.playerId}>
                <TableCell className={nameClass}>{name}</TableCell>
                <TableCell className="text-muted-foreground">
                  {getMechaName(p.mechaId)}
                </TableCell>
                <TableCell className="text-muted-foreground">{ai === true ? "Yes" : ai === false ? "No" : "-"}</TableCell>
                <TableCell className="text-muted-foreground">
                  {ready === true ? "Yes" : ready === false ? "No" : "-"}
                </TableCell>
              </TableRow>
            );
          })}
          {players.length === 0 ? (
            <TableRow>
              <TableCell colSpan={4} className="text-muted-foreground">
                No players yet.
              </TableCell>
            </TableRow>
          ) : null}
        </TableBody>
      </Table>
    </div>
  );
}

export default function App() {
  const tracker = useTracker();
  const teams = tracker.snapshot?.teams ?? { team1: [], team2: [], unassigned: [] };

  return (
    <Shell>
      <Tabs defaultValue="roster">
        <TabsList>
          <TabsTrigger value="roster">Roster</TabsTrigger>
          <TabsTrigger value="settings">Settings</TabsTrigger>
          <TabsTrigger value="about">About</TabsTrigger>
        </TabsList>

        <TabsContent value="roster">
          <div className="grid grid-cols-1 gap-4 md:grid-cols-2">
            <TeamTable title="Team 1" players={teams.team1} />
            <TeamTable title="Team 2" players={teams.team2} />
          </div>
          {teams.unassigned.length ? (
            <div className="mt-4">
              <TeamTable title="Unassigned" players={teams.unassigned} />
            </div>
          ) : null}
        </TabsContent>

        <TabsContent value="settings">
          <div className="rounded-xl border border-border bg-card/60 p-4 backdrop-blur">
            <div className="mb-2 text-sm font-semibold">Settings</div>
            <SettingsPanel />
          </div>
        </TabsContent>

        <TabsContent value="about">
          <div className="rounded-xl border border-border bg-card/60 p-4 backdrop-blur">
            <div className="mb-2 text-sm font-semibold">About</div>
            <About />
          </div>
        </TabsContent>
      </Tabs>
    </Shell>
  );
}

function SettingsPanel() {
  const cfg = useConfig();

  return (
    <div className="space-y-4">
      <div className="text-sm text-muted-foreground">
        Configure log source. Manual mode is useful if process auto-detection fails.
      </div>

      <div className="grid grid-cols-1 gap-3 md:grid-cols-2">
        <label className="space-y-1">
          <div className="text-xs font-medium text-muted-foreground">Mode</div>
          <select
            className="h-10 w-full rounded-md border border-input bg-background/30 px-3 text-sm outline-none focus:ring-2 focus:ring-ring"
            value={cfg.config.mode}
            onChange={(e) => cfg.setMode(e.target.value as any)}
            disabled={!cfg.isTauri}
          >
            <option value="Auto">Auto</option>
            <option value="Manual">Manual</option>
          </select>
        </label>

        <label className="space-y-1">
          <div className="text-xs font-medium text-muted-foreground">Manual log path</div>
          <input
            className="h-10 w-full rounded-md border border-input bg-background/30 px-3 text-sm outline-none focus:ring-2 focus:ring-ring"
            placeholder="C:\\path\\to\\MechaBREAK\\logs\\...\\latest.log"
            value={cfg.config.manualLogPath ?? ""}
            onChange={(e) => cfg.setManualLogPath(e.target.value)}
            disabled={!cfg.isTauri || cfg.config.mode !== "Manual"}
          />
        </label>
      </div>

      <div className="flex items-center gap-2">
        <Button
          variant="outline"
          disabled={!cfg.isTauri}
          onClick={cfg.save}
          title={!cfg.isTauri ? "Run via Tauri to save settings" : undefined}
        >
          Save
        </Button>
        <div className="text-xs text-muted-foreground">
          {cfg.isTauri ? "Saved to your app data folder." : "Settings are disabled in web preview."}
        </div>
      </div>
    </div>
  );
}

