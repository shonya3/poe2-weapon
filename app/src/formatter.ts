const formatter = Intl.NumberFormat('en', { maximumFractionDigits: 1 });
export function fmt(num: number): string {
	return formatter.format(num);
}
