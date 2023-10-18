// place files you want to import through the `$lib` alias in this folder.
export interface WindowResize {
	width: number;
	height: number;
}
export interface OhlcEventPayload {
	time: number;
	open: number;
	high: number;
	low: number;
	close: number;
	vwap: number;
	volume: number;
	count: number;
}

export type ChartDataItem = {
	time: string;
	open: number;
	high: number;
	low: number;
	close: number;
};

export type TradeBalance = {
	equivalent?: number | null;
	trade?: number | null;
	margin?: number | null;
	unrealized_pl?: number | null;
	cost?: number | null;
	valuation?: number | null;
	equity?: number | null;
	free_magin?: number | null;
	margin_level?: number | null;
	unexecuted_value?: number | null;
};

export type ExtendedBalance = {
	balance: number;
	hold_trade: number;
};

export type ExtendedBalances = {
	[key: string]: ExtendedBalance;
};

// export function isExtendedBalances(data: any): data is ExtendedBalances {
// 	const potentialBalance = (data as ExtendedBalances)[Object.keys(data)[0]];
// 	return data && typeof data === 'object' && 'balance' in potentialBalance;
// }

// export function isTradeBalance(data: any): data is TradeBalance {
// 	return data && typeof data === 'object' && 'equivalent' in data;
// }

// export function mapToChartFormat(
// 	data: Array<OhlcEventPayload['message'][number]>
// ): ChartDataItem[] {
// 	return data.map((item) => {
// 		const dateObj = new Date(item.time * 1000); // Convert UNIX timestamp to Date object

// 		const year = dateObj.getUTCFullYear();
// 		const month = String(dateObj.getUTCMonth() + 1).padStart(2, '0'); // Months are 0-based
// 		const day = String(dateObj.getUTCDate()).padStart(2, '0');

// 		return {
// 			time: `${year}-${month}-${day}`,
// 			open: item.open,
// 			high: item.high,
// 			low: item.low,
// 			close: item.close
// 		};
// 	});
// }
