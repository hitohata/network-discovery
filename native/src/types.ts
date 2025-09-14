export type NodeOverviewType = {
	ip: string;
	machineInfo: {
		os: string;
		os_version: string;
		hostName: string;
		kernelVersion: string;
		numberOfCpu: number;
		arch: string;
		brand: string;
	} | null;
	usage: {
		totalMemory: number;
		usedMemory: number;
		totalSwap: number;
		usedSwap: number;
		cpuUsage: number[];
		cpuFrequency: number[];
		networkDown: number;
		networkUp: number;
	} | null;
	lastUpdated: string;
};
