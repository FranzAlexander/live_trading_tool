export type RangeBar = {
	open: number;
	high: number;
	low: number;
	close: number;
	start_time: number;
	is_first_bar: boolean;
};

export type DeltaBar = {
	close: number;
	high: number;
	low: number;
	open: number;
	delta: number;
};
