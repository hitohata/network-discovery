import { invoke } from "@tauri-apps/api/core";
import { useEffect, useState } from "react";
import type { NodeOverviewType } from "./types";
import "./App.css";
import { NodeOverview } from "./components/Overview.tsx";

function App() {
	const [nodeOverview, setNodeOverview] = useState<NodeOverviewType[]>([]);

	useEffect(() => {
		const getNodeOverview = async () => {
			setNodeOverview(await invoke("get_nodes"));
		};
		getNodeOverview().catch((e) => console.error(e));
	}, []);

	return (
		<main className="container">
			{nodeOverview.map((overview) => NodeOverview(overview))}
		</main>
	);
}

export default App;
