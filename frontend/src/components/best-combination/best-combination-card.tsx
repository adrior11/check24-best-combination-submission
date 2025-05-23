import React from 'react';
import { MdOutlineCancel, MdCheckCircleOutline } from 'react-icons/md';

import { formatPrice } from './util';
import type { BestCombination } from './types';

// Helper function for coverage indicators
const coverageIndicator = (value: number) => {
    // 0 = grey cancel, 1 = yellow check, 2 = green check
    switch (value) {
        case 0:
            return (
                <div className="inline-block text-gray-400 flex items-center justify-center" title="No coverage">
                    <MdOutlineCancel />
                </div>
            );
        case 1:
            return (
                <div className="inline-block text-yellow-400 flex items-center justify-center" title="Partial coverage">
                    <MdCheckCircleOutline />
                </div>
            );
        case 2:
            return (
                <div className="inline-block text-green-500 flex items-center justify-center" title="Full coverage">
                    <MdCheckCircleOutline />
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
    const coverageKeys = React.useMemo(() => {
        const keysSet = new Set<string>();
        combination.packages.forEach(pkg => {
            Object.keys(pkg.coverage).forEach(key => keysSet.add(key));
        });
        // Convert Set to Array and sort alphabetically
        return Array.from(keysSet).sort((a, b) => a.localeCompare(b));
    }, [combination.packages]);

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
                        &nbsp;| Yearly:{' '}
                        <strong>{formatPrice(combination.combinedMonthlyPriceYearlySubscriptionInCents)}</strong>
                    </div>
                </div>
            </div>

            {/* Coverage Matrix */}
            <div className="overflow-x-auto">
                <table className="min-w-full border-collapse text-sm">
                    <thead className="align-bottom">
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
                                        {/* Render only if monthlyPriceCents exists */}
                                        {pkg.monthlyPriceCents !== undefined && (
                                            <span className="text-xs text-gray-500">
                                                {formatPrice(pkg.monthlyPriceCents)} pm
                                            </span>
                                        )}
                                        <span className="text-xs text-gray-500">
                                            {formatPrice(pkg.monthlyPriceYearlySubscriptionInCents)} pm (yr)
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
