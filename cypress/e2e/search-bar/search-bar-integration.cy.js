suite(["@CI"],'search bar',() => {

    let state_hash="CkpYfTKJyVjWmM5Lb5SdzRL6GuEbJf6q7yYAyW6NkvkYFZQaY5PGz";
    let block_hash="3NLqPGGVtxXdsQg2orrp3SFFE3ToeMuqWRerSRWbmAKuSk2tphWy";
    let public_key="B62qrQKS9ghd91shs73TCmBJRW9GzvTJK443DPx2YbqcyoLc56g1ny9";

    let pages = [
        { origin: "/", input: block_hash, tableHeading: 'Blocks' },
        { origin: "/summary", input: block_hash, tableHeading: 'Blocks' },
        { origin: "/blocks", input: block_hash, tableHeading: 'Blocks' },
        { origin: "/transactions", input: state_hash, tableHeading:'Transactions'},
        { origin: "/stakes", input: public_key, tableHeading:'Current Staking Ledger'},
        { origin: "/next-stakes", input: public_key, tableHeading:'Next Staking Ledger'},
    ];

    it('works on /snarks page', () => {
        let prover = "B62qnM71LjMchDsRgWinBXyNrXR8smf9NXoJZnQrTXe74DrEQoaUStb";
        let tableHeading = 'SNARKs';
        let tableColumn = 'Prover';
        cy.visit('/snarks');
        cy.wait(1000);
        cy.get("input#searchbar").type(prover, {delay:0})
        cy.tableColumnValuesEqual(tableHeading, tableColumn, prover)
    })

    pages.forEach(({origin, input, tableHeading}) => it(`works on ${origin} page`, () => {
        cy.visit(origin);
        cy.wait(1000);
        cy.get("input#searchbar").as('searchinput');
        cy.get("@searchinput").type(input, {delay:0});

        // check input
        cy.get("@searchinput").should('have.value', input);
        // check url
        cy.url().should('include', `query=${input}`);
        // check table
        cy.tableHasNRows(tableHeading, 1);
        cy.wait(1000);

        cy.go('back');

        // check input
        cy.get("@searchinput").should('have.value', '');
        // check url
        cy.url().should('not.contain', `query`);
        // check table
        cy.tableHasNRows(tableHeading, 10);
    }));

})
