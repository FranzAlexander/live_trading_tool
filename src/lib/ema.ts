import type { OhlcEventPayload } from '$lib';
import { invoke } from '@tauri-apps/api/tauri';

export type EmaHistory = {
	time: number;
	value: number;
};

export async function setEmaHistory(ohlc: OhlcEventPayload[]): Promise<EmaHistory[]> {
	return await invoke('ema_history', { ohlcs: ohlc });
}
