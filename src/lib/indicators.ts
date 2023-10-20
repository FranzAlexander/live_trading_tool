import type { OhlcEventPayload } from '$lib';
import { invoke } from '@tauri-apps/api/tauri';

export type MacdHistory = {
	time: number;
	macd: number;
	signal: number;
	histogram: number;
};

export async function setMacdHistory(ohlc: OhlcEventPayload[]): Promise<MacdHistory[]> {
	return await invoke('macd_history', { ohlcs: ohlc });
}
