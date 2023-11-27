import { component$, useResource$, Resource } from "@builder.io/qwik";
import InfoSection from "./info-section";


export default component$(() => {

    const summaryResource = useResource$(async () => {
        const res = await fetch('https://api.minaexplorer.com/summary');
        const data = res.json();
        return data;
    })

    return (
        <>
            <h1>Summary</h1>
            <section class="summary-section">
                <Resource
                    value={summaryResource}
                    onResolved={(summary) => (
                        <>
                            <InfoSection label="Height" value={summary.blockchainLength} />
                            <InfoSection label="Circulating Supply" value={summary.circulatingSupply} />
                            <InfoSection label="Epoch" value={summary.epoch} />
                            <InfoSection label="Slot" value={summary.slot} />
                            <InfoSection label="Total Currency" value={summary.totalCurrency} />
                        </>
                    )}
                />
            </section>
        </>
    );
});