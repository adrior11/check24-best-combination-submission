export const DEFAULT_LIMIT = 3; // must be between [1, 5]

export interface Coverage {
    [key: string]: number[];
}

export interface Package {
    name: string;
    coverage: Coverage;
    monthlyPriceCents: number | undefined;
    monthlyPriceYearlySubscriptionInCents: number;
}

export interface BestCombination {
    combinedCoverage: number;
    combinedMonthlyPriceCents: number;
    combinedMonthlyPriceYearlySubscriptionInCents: number;
    packages: Package[];
}

export enum BestCombinationStatus {
    READY = 'READY',
    PROCESSING = 'PROCESSING',
    ERROR = 'ERROR',
}

export interface BestCombinationResponse {
    status: BestCombinationStatus;
    data: BestCombination[] | undefined;
}
