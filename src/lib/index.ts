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

export type ExtendedDetails = {
	balance: number;
	hold_trade: number;
};

export type ExtendedBalances = {
	[key: string]: ExtendedDetails;
};

export type CoinName = 'BTC';

export const coinIcons: Record<CoinName, unknown> = {
	BTC
};

export type OrderType =
	| 'market'
	| 'limit'
	| 'stop-loss'
	| 'take-profit'
	| 'stop-loss-limit'
	| 'take-profit-limit'
	| 'settle-position';

export type OrderOption = 'spot' | 'margin';

export type BuySell = 'buy' | 'sell';

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

export type Fee = {
	fee: string;
	volume: number;
};

type AssetClass = 'currency';

type PairStatus = 'online' | 'offline'; // Assuming 'status' can have a finite set of values

export type TradingPairInfo = {
	aclass_base: AssetClass;
	aclass_quote: AssetClass;
	altname: string;
	base: string;
	cost_decimals: number;
	costmin: string;
	fee_volume_currency: string;
	fees: Fee[];
	fees_maker: Fee[];
	leverage_buy: number[];
	leverage_sell: number[];
	long_position_limit: number;
	lot: string;
	lot_decimals: number;
	lot_multiplier: number;
	margin_call: number;
	margin_stop: number;
	ordermin: string;
	pair_decimals: number;
	quote: string;
	short_position_limit: number;
	status: PairStatus;
	tick_size: string;
	wsname: string;
};
