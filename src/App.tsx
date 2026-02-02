import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { Button } from "@/components/ui/button";
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from "@/components/ui/table";
import { useTracker } from "@/hooks/useTracker";
import { useConfig } from "@/hooks/useConfig";
import type { Player } from "@/types";
import { About } from "@/components/About";
import { getMechaName } from "@/lib/mechaNames";
import { cn } from "@/lib/utils";

import { TitleBar } from "@/components/TitleBar";

function Shell({ children }: { children: React.ReactNode }) {
  const tracker = useTracker();
  return (
    <div className="flex h-screen flex-col overflow-hidden bg-background text-foreground">
      <TitleBar />
      <div className="flex-1 overflow-y-auto no-scrollbar">
        <div className="mx-auto flex max-w-7xl flex-col gap-6 px-4 py-6 sm:px-8">
          <header className="flex items-center justify-between">
            <div className="flex flex-col">
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
    </div>
  );
}

interface TeamTableProps {
  title: string;
  players: Player[];
  variant?: "blue" | "red" | "neutral";
}

function TeamTable({ title, players, variant = "neutral" }: TeamTableProps) {
  const variants = {
    blue: {
      container: "from-blue-500/10 to-transparent border-blue-500/20 shadow-[0_0_15px_-3px_rgba(59,130,246,0.3)]",
      header: "border-blue-500/20 bg-blue-500/5 text-blue-100",
      title: "text-blue-400",
      row: "hover:bg-blue-500/5",
    },
    red: {
      container: "from-red-600/10 to-transparent border-red-600/20 shadow-[0_0_15px_-3px_rgba(220,38,38,0.3)]",
      header: "border-red-600/20 bg-red-600/5 text-red-100",
      title: "text-red-400",
      row: "hover:bg-red-600/5",
    },
    neutral: {
      container: "bg-card/60 backdrop-blur border-border",
      header: "",
      title: "text-foreground",
      row: "",
    },
  };

  const theme = variants[variant];

  // For blue/red, we use a gradient background. For neutral, we keep the original card look.
  const containerClass = cn(
    "rounded-xl border p-4 transition-all duration-300",
    variant !== "neutral" ? "bg-gradient-to-b backdrop-blur-sm" : "",
    theme.container
  );

  return (
    <div className={containerClass}>
      <div className="mb-3 flex items-baseline justify-between px-1">
        <div className={cn("text-sm font-semibold uppercase tracking-wider", theme.title)}>
          {title}
        </div>
        <div className="text-xs text-muted-foreground">{players.length} players</div>
      </div>
      <div className="overflow-hidden rounded-lg border border-border/50">
        <Table>
          <TableHeader className={cn("bg-muted/50", theme.header)}>
            <TableRow className="border-border/50 hover:bg-transparent">
              <TableHead className="w-[30%]">Player</TableHead>
              <TableHead className="w-[30%]">Mecha</TableHead>
              <TableHead className="w-[20%] text-center">AI</TableHead>
              <TableHead className="w-[20%] text-right">Ready</TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            {players.map((p) => {
              const name = p.displayName ?? `Player ${p.playerId}`;
              const ai = p.isAi;
              const ready = p.ready;
              // Specific styling for ready/AI status
              const nameClass =
                ai === true
                  ? "text-red-300 font-medium"
                  : ai === undefined
                    ? "text-yellow-200"
                    : "text-foreground font-medium";

              return (
                <TableRow key={p.playerId} className={cn("border-border/50", theme.row)}>
                  <TableCell className={nameClass}>{name}</TableCell>
                  <TableCell className="text-muted-foreground">
                    {getMechaName(p.mechaId)}
                  </TableCell>
                  <TableCell className="text-muted-foreground text-center">
                    {ai === true ? "Yes" : ai === false ? "No" : "-"}
                  </TableCell>
                  <TableCell className="text-right">
                    {ready === true ? (
                      <span className="inline-flex items-center rounded-sm bg-green-500/10 px-2 py-0.5 text-xs font-medium text-green-400 ring-1 ring-inset ring-green-500/20">
                        READY
                      </span>
                    ) : ready === false ? (
                      <span className="text-muted-foreground">-</span>
                    ) : (
                      "-"
                    )}
                  </TableCell>
                </TableRow>
              );
            })}
            {players.length === 0 ? (
              <TableRow className="hover:bg-transparent">
                <TableCell colSpan={4} className="h-24 text-center text-muted-foreground">
                  Waiting for players...
                </TableCell>
              </TableRow>
            ) : null}
          </TableBody>
        </Table>
      </div>
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
            <TeamTable title="Team 1" players={teams.team1} variant="blue" />
            <TeamTable title="Team 2" players={teams.team2} variant="red" />
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

