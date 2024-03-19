suite(["@CI"],'meta title',() => {

    let public_key="B62qpge4uMq4Vv5Rvc8Gw9qSquUYd6xoW1pz7HQkMSHm6h1o7pvLPAN";
    let block_hash="3NLhBh3d4b91DPoJn5hhwRAWmHSAaG8Qz4W5r9FhJBCXLD3WrAt4";
    let pages = [
        { url: `/addresses/accounts`, title: `Accounts | Search For Mina Account`},
        { url: '/blocks', title: 'Blocks | Search for Mina Blocks'},
        { url: `/blocks/${block_hash}/`, title: 'Block Overview | Spotlight'},
        { url: `/blocks/${block_hash}/user-commands`, title: 'Block Overview | User Commands'},
        { url: `/blocks/${block_hash}/internal-commands`, title: 'Block Overview | Internal Commands'},
        { url: `/blocks/${block_hash}/snark-jobs`, title: 'Block Overview | SNARK Jobs'},
        { url: '/broadcast/transaction', title: 'Offline Broadcasting | Broadcast Signed Transaction'},
        { url: '/broadcast/delegation', title: 'Offline Broadcasting | Broadcast Signed Delegation'},
        { url: '/broadcast/ledger', title: 'Offline Broadcasting | Broadcast Signed Transaction From Ledger'},
        { url: '/next-stakes', title: 'Next Staking Ledger | Search For Stakers'},
        { url: '/stakes', title: 'Staking Ledger | Current Staking Ledger'},
        { url: '/stakes?epoch=71', title: 'Staking Ledger | Epoch 71'},
        { url: '/snarks', title: 'SNARKs | Search For SNARKs'},
        { url: '/', title: 'Mina Blockchain Explorer | Search For Blocks'},
        { url: '/summary', title: 'Mina Blockchain Explorer | Search For Blocks'},
        { url: `/addresses/accounts/${public_key}`, title: `Account Overview | 'MinaExplorer'`},
    ];

    pages.forEach(({title, url}) => it(`'${title}' exists in <head> for page ${url}`, () => {
        cy.visit(url);
        cy.title({timeout: 20000}).should('equal', title);
    }));

})
