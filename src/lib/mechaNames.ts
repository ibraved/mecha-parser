// Mecha ID to name mapping from the game
export const MECHA_NAMES: Record<number, string> = {
    100001: "Falcon",
    100002: "Panther",
    100003: "Alysnes",
    100004: "Tricera",
    100005: "Narukami",
    100006: "Serenith",
    100007: "Luminae",
    100008: "Pinaka",
    100009: "Inferno",
    100010: "Skyraider",
    100011: "Norne",
    100012: "Welkin",
    100015: "Aquila",
    100016: "Stego",
    100017: "Stellaris",
    100018: "Hurricane",
    100019: "Alphard",
    100020: "Freyr",
    100021: "Hel",
    100023: "Mikillja",
};

export function getMechaName(mechaId: number | undefined): string {
    if (!mechaId) return "-";
    return MECHA_NAMES[mechaId] ?? `Unknown(${mechaId})`;
}
