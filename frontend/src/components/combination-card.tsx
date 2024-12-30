import React, { useState } from 'react';

// Helper function for coverage indicators
const coverageIndicator = (value: number) => {
    // 0 = grey X, 1 = yellow check, 2 = green check
    switch (value) {
        case 0:
            return (
                <div
                    className="inline-block w-4 h-4 rounded-full bg-gray-400 text-white flex items-center justify-center"
                    title="No coverage"
                >
                    ✕
                </div>
            );
        case 1:
            return (
                <div
                    className="inline-block w-4 h-4 rounded-full bg-yellow-400 text-white flex items-center justify-center"
                    title="Partial coverage"
                >
                    ✓
                </div>
            );
        case 2:
            return (
                <div
                    className="inline-block w-4 h-4 rounded-full bg-green-500 text-white flex items-center justify-center"
                    title="Full coverage"
                >
                    ✓
                </div>
            );
        default:
            return <div />;
    }
};

interface Coverage {
    [key: string]: number[];
}

interface Package {
    name: string;
    coverage: Coverage;
    monthlyPriceCents: number | undefined;
    monthlyPriceYearlySubscriptionInCents: number;
}

interface BestCombination {
    combinedCoverage: number;
    combinedMonthlyPriceCents: number;
    combinedMonthlyPriceYearlySubscriptionInCents: number;
    packages: Package[];
}

interface CombinationCardProps {
    combination: BestCombination;
    index: number;
}

export const CombinationCard: React.FC<CombinationCardProps> = ({ combination, index }) => {
    const [isExpanded, setIsExpanded] = useState(false);

    // In your example, each combination has multiple packages, each with coverage.
    // We want a unique set of coverage “keys” across all packages to build our matrix row labels.
    const coverageKeys = Array.from(new Set(combination.packages.flatMap(pkg => Object.keys(pkg.coverage))));

    // Convert prices from cents to euros as needed
    const monthlyPriceInEuros = (combination.combinedMonthlyPriceCents / 100).toFixed(2);
    const yearlyPriceInEuros = (combination.combinedMonthlyPriceYearlySubscriptionInCents / 100).toFixed(2);

    return (
        <div className="border rounded-md p-4 mb-4">
            {/* High-Level Header */}
            <div className="flex justify-between items-center">
                <div>
                    {/* For example: "Combination #1 (2 packages)" */}
                    <h3 className="font-bold text-lg mb-1">
                        Combination #{index + 1} ({combination.packages.length} package
                        {combination.packages.length > 1 ? 's' : ''})
                    </h3>
                    <div className="text-sm text-gray-700">
                        Coverage: <strong>{combination.combinedCoverage}%</strong>
                        &nbsp;| Monthly: <strong>€{monthlyPriceInEuros}</strong>
                        &nbsp;| Yearly: <strong>€{yearlyPriceInEuros}</strong>
                    </div>
                </div>

                {/* Toggle button */}
                <button onClick={() => setIsExpanded(!isExpanded)} className="text-blue-500 text-sm hover:underline">
                    {isExpanded ? 'Hide details' : 'Show details'}
                </button>
            </div>

            {/* Expanded coverage matrix */}
            {isExpanded && (
                <div className="mt-4 overflow-x-auto">
                    {/* Package names as columns */}
                    <table className="min-w-full border-collapse text-sm">
                        <thead>
                            <tr>
                                <th className="p-2 text-left border-b border-gray-200">Coverage</th>
                                {combination.packages.map((pkg, i) => (
                                    <th key={i} className="p-2 text-center border-b border-gray-200">
                                        {pkg.name}
                                        <br />
                                        <span className="text-xs text-gray-500">
                                            €{(pkg.monthlyPriceCents ?? 0 / 100).toFixed(2)}/mo
                                        </span>
                                    </th>
                                ))}
                            </tr>
                        </thead>
                        <tbody>
                            {coverageKeys.map(key => {
                                // We will place 2 indicators (live vs highlight) side by side
                                // inside a single cell for each package
                                return (
                                    <tr key={key} className="border-b last:border-none">
                                        <td className="p-2 border-gray-200">{key}</td>

                                        {combination.packages.map((pkg, i) => {
                                            const coverageArray = pkg.coverage[key];
                                            // Typically coverageArray = [liveValue, highlightValue]
                                            const [liveValue, highlightValue] = coverageArray || [0, 0];
                                            return (
                                                <td key={i} className="p-2 text-center border-gray-200">
                                                    <div className="flex flex-col gap-1 items-center justify-center">
                                                        <div>{coverageIndicator(liveValue)}</div>
                                                        <div>{coverageIndicator(highlightValue)}</div>
                                                    </div>
                                                </td>
                                            );
                                        })}
                                    </tr>
                                );
                            })}
                        </tbody>
                    </table>
                </div>
            )}
        </div>
    );
};
