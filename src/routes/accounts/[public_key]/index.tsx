import { component$, useResource$, Resource } from "@builder.io/qwik";
import { Link, useLocation } from '@builder.io/qwik-city';
import InfoSection from "../../../components/info-section/info-section";

export default component$(() => {

    const loc = useLocation();

    const accountsResource = useResource$(async () => {
        const res = await fetch(`https://api.minaexplorer.com/accounts/${loc.params.public_key}`);
        const data = res.json();
        return data;
    });

    return (
        <>
            <h1>Account</h1>
            <section class="accounts-section">
                <Resource
                    value={accountsResource}
                    onResolved={(res) => (
                        <>
                            <InfoSection label="Public Key" value={res.account.publicKey} />
                            <InfoSection label="Username" value={res.account.username} />
                            <InfoSection label="Balance" value={res.account.balance.total} />
                            <InfoSection label="Nonce" value={res.account.nonce} />
                            <InfoSection label="Receipt Chain Hash" value={res.account.receiptChainHash} />
                            <InfoSection label="Delegate">
                                <Link href={`/accounts/${res.account.delegate}/`}>{res.account.delegate}</Link>
                            </InfoSection>
                            <InfoSection label="Voting For" value={res.account.votingFor} />
                            <InfoSection label="Total Transactions">
                                <Link href={`/transactions/${res.account.publicKey}`}>{res.account.totalTx}</Link>
                            </InfoSection>
                        </>
                    )}
                />
            </section>
        </>
    );
});