import type { NodeOverviewType } from "../types.ts";

export const NodeOverview = (overview: NodeOverviewType) => {
	return (
		<div className="node-overview">
			<h2>{overview.ip}</h2>
			<h3>OS</h3>
			{overview.machineInfo && (
				<div>
					<p>
						{overview.machineInfo?.os} {overview.machineInfo?.os_version}
					</p>
					<p>Hostname: {overview.machineInfo?.hostName}</p>
					<p>Kernel: {overview.machineInfo?.kernelVersion}</p>
					<p>
						CPU: {overview.machineInfo?.brand} (
						{overview.machineInfo?.numberOfCpu} cores,{" "}
						{overview.machineInfo?.arch})
					</p>
				</div>
			)}
			<h3>Usage</h3>
			{overview.usage && (
				<div>
					<p>
						Memory: {overview.usage.usedMemory} / {overview.usage.totalMemory}{" "}
						MB
					</p>
					<p>
						Swap: {overview.usage.usedSwap} / {overview.usage.totalSwap} MB
					</p>
					<p>CPU Usage: {overview.usage.cpuUsage.join(", ")} %</p>
					<p>CPU Frequency: {overview.usage.cpuFrequency.join(", ")} MHz</p>
					<p>Network Down: {overview.usage.networkDown} bytes</p>
					<p>Network Up: {overview.usage.networkUp} bytes</p>
				</div>
			)}
		</div>
	);
};
