import { formatPrice } from './util';
import type { BestCombination } from './types';

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

interface CombinationCardProps {
    combination: BestCombination;
    index: number;
}

export const CombinationCard: React.FC<CombinationCardProps> = ({ combination, index }) => {
    // Construct a unique set of coverage keys across all packages to build matrix row labels.
    const coverageKeys = Array.from(new Set(combination.packages.flatMap(pkg => Object.keys(pkg.coverage))));

    return (
        <div className="relative border-2 rounded-md p-6 mb-4">
            {/* Badge for Best Combination */}
            {index === 0 && (
                <div className="absolute top-0 right-5 -translate-y-1/2 rounded-full border-2 bg-default text-xs font-bold px-3 py-1">
                    <span className="gradient-text">Best Combination</span>
                </div>
            )}

            {/* High-Level Header */}
            <div className="flex justify-between items-center mb-4">
                <div>
                    <div className="text-sm text-gray-500">
                        <strong>
                            {combination.packages.length} package
                            {combination.packages.length > 1 ? 's' : ''}
                        </strong>
                        &nbsp;| Coverage: <strong>{combination.combinedCoverage}%</strong>
                        &nbsp;| Monthly: <strong>{formatPrice(combination.combinedMonthlyPriceCents)}</strong>
                        &nbsp;| Yearly:
                        <strong>{formatPrice(combination.combinedMonthlyPriceYearlySubscriptionInCents)}</strong>
                    </div>
                </div>
            </div>

            {/* Coverage Matrix */}
            <div className="overflow-x-auto">
                <table className="min-w-full border-collapse text-sm">
                    <thead>
                        {/* Header Row: Package Names */}
                        <tr>
                            <th className="p-2 text-left border-b border-gray-200"></th>
                            {combination.packages.map((pkg, i) => (
                                <th key={i} className="p-2 text-center border-b border-gray-200">
                                    {pkg.name}
                                    <div className="mt-1 flex flex-row items-center justify-center space-x-2">
                                        <span className="text-xs px-1.5 py-0.5 text-gray-500 rounded-full bg-offset">
                                            Live
                                        </span>
                                        <span className="text-xs px-1.5 py-0.5 text-gray-500 rounded-full bg-offset">
                                            Highl.
                                        </span>
                                    </div>
                                </th>
                            ))}
                        </tr>
                    </thead>
                    <tbody>
                        {coverageKeys.map(key => (
                            <tr key={key} className="border-b">
                                <td className="p-2 border-gray-200">{key}</td>
                                {combination.packages.map((pkg, i) => {
                                    const coverageArray = pkg.coverage[key];
                                    const [liveValue, highlightValue] = coverageArray || [0, 0];
                                    return (
                                        <td key={i} className="p-2 text-center border-gray-200">
                                            <div className="flex flex-row gap-3 items-center justify-center">
                                                <div>{coverageIndicator(liveValue)}</div>
                                                <div>{coverageIndicator(highlightValue)}</div>
                                            </div>
                                        </td>
                                    );
                                })}
                            </tr>
                        ))}
                    </tbody>
                    <tfoot>
                        {/* Footer Row: Package Prices */}
                        <tr>
                            <th className="p-2 text-left"></th>
                            {combination.packages.map((pkg, i) => (
                                <th key={i} className="p-2 text-center">
                                    <div className="flex flex-col items-center">
                                        <span className="text-xs text-gray-500">
                                            {formatPrice(pkg.monthlyPriceCents ?? 0)}/m
                                        </span>
                                        <span className="text-xs text-gray-500">
                                            {formatPrice(pkg.monthlyPriceYearlySubscriptionInCents)}/y
                                        </span>
                                    </div>
                                </th>
                            ))}
                        </tr>
                    </tfoot>
                </table>
            </div>
        </div>
    );
};
