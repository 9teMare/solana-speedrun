export interface EventData {
    data: Record<string, any>;
    name: string;
}

export enum Race {
    ROYAL = "ROYAL",
    HUMANOID = "HUMANOID",
    UNDEAD = "UNDEAD",
}
