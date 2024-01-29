suite(["@CI"],'search bar',() => {

    let payment_id="4Rs6xMHVyo1HmXm4m5azosTMSvbEJDkDAs6tYoCc6wTPz8YHJPNFMoCi117YcTdHd46hMaCQs6BuGZtxHntoqJPdpLZae6o7BXFGaRmBXHdZDEn6UpLmZR5sNzGujR2VwMAi3nvymfcLQkKM1Qxdy62NgPe2p3pKvnJsdUmkAX1ERuejKaRz9JyBrUgc7i1WoEx2Etp1tKg4pnRQe3CpKHuYCvEbV4xUh7Gkm8TqgZYCfkU9crZxA5BmucJ8A1RU1gBqv2pNPhaWQ2ExSFuBqGmYrSZ6nqcU6LhfHZirjZd6eC673Wn9CL5txSpnsqd61HUgoiLjpLUQ3Sdg3F8C6s2D1xuRhhqXG8E2xxsBjYS3B5oHLndrEb7WaqJY5YULNGHq6UVk";
    let block_hash="3NLqPGGVtxXdsQg2orrp3SFFE3ToeMuqWRerSRWbmAKuSk2tphWy";
    let public_key="B62qrQKS9ghd91shs73TCmBJRW9GzvTJK443DPx2YbqcyoLc56g1ny9";

    let pages = [
        { origin: "/", input: block_hash, tableHeading: 'Blocks' },
        { origin: "/summary", input: block_hash, tableHeading: 'Blocks' },
        { origin: "/blocks", input: block_hash, tableHeading: 'Blocks' },
        { origin: "/transactions", input: payment_id, tableHeading:'Transactions'},
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
        cy.get("input#searchbar").type(input, {delay:0});
        cy.tableHasNRows(tableHeading, 1)
    }));

})
