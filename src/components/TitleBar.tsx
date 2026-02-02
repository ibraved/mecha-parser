import { useState, useEffect } from "react";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { Minus, Square, X, Copy } from "lucide-react"; // Import Copy icon as placeholder for restore if needed, or just use Square
import { cn } from "@/lib/utils";
import logo from "@/assets/logo.png";

export function TitleBar() {
    const [isMaximized, setIsMaximized] = useState(false);
    const appWindow = getCurrentWindow();

    useEffect(() => {
        const updateState = async () => {
            setIsMaximized(await appWindow.isMaximized());
        };
        updateState();

        const unlisten = appWindow.onResized(updateState);
        return () => {
            unlisten.then(f => f());
        }
    }, []);

    const minimize = () => appWindow.minimize();
    const toggleMaximize = async () => {
        await appWindow.toggleMaximize();
        setIsMaximized(await appWindow.isMaximized());
    };
    const close = () => appWindow.close();

    return (
        <div data-tauri-drag-region className="flex h-10 w-full select-none items-center justify-between bg-background/50 pl-4 backdrop-blur-md border-b border-white/5">
            <div className="flex items-center gap-2 text-xs font-bold uppercase tracking-widest text-muted-foreground pointer-events-none">
                <img src={logo} alt="Logo" className="h-4 w-4 opacity-70" />
                <span>Mecha Parser</span>
            </div>

            <div className="flex h-full">
                <button
                    onClick={minimize}
                    className="flex h-full w-12 items-center justify-center hover:bg-white/10 active:bg-white/20"
                >
                    <Minus className="h-4 w-4" />
                </button>
                <button
                    onClick={toggleMaximize}
                    className="flex h-full w-12 items-center justify-center hover:bg-white/10 active:bg-white/20"
                >
                    {isMaximized ? (
                        <Copy className="h-3 w-3 rotate-180" />
                    ) : (
                        <Square className="h-3 w-3" />
                    )}
                </button>
                <button
                    onClick={close}
                    className="flex h-full w-12 items-center justify-center hover:bg-red-500 hover:text-white active:bg-red-600"
                >
                    <X className="h-4 w-4" />
                </button>
            </div>
        </div>
    );
}
