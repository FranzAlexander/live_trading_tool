import type { OhlcEventPayload } from '$lib';
import { invoke } from '@tauri-apps/api/tauri';

export type Ema = {
	period: number;
	multiplier: number;
	current: number;
	couunt: number;
};

export type EmaHistory = {
	time: number;
	value: number;
};

export async function createEma(period: number): Promise<Ema> {
	return await invoke('create_ema', { period: period });
}

export async function setEmaHistory(ema: Ema, ohlc: OhlcEventPayload[]): Promise<EmaHistory[]> {
	return await invoke('ema_history', { period: ema.period, multi: ema.multiplier, ohlcs: ohlc });
}
