// Convert prices from cents to euros as needed
export const formatPrice = (priceInCents: number, currency: string = '€'): string => {
    return `${(priceInCents / 100).toFixed(2)}${currency}`;
};
