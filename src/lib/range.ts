export type RangeBar = {
	close: number;
	high: number;
	is_new_bar: boolean;
	low: number;
	open: number;
	start_time: number;
	volume: number;
};

export type DeltaBar = {
	close: number;
	high: number;
	low: number;
	open: number;
	delta: number;
	start_time: number;
	is_first_bar: boolean;
};

export type RangeBarPayload = {
	rangeBar: RangeBar;
	deltaBar: DeltaBar;
};
