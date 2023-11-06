import BTC from './icons/BTC.svelte';

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

export type CoinName = 'BTC';

export const coinIcons: Record<CoinName, unknown> = {
	BTC
};

export type OrderType = 'buy' | 'sell';

export type OhlcPayload = {
	name: string;
	time: number;
	etime: number;
	open: number;
	high: number;
	low: number;
	close: number;
	vwap: number;
	volume: number;
};
