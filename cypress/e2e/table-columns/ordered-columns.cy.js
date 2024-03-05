suite(["@CI"],'transactions table', () => {

    let pages = ['/transactions'];
    let columns = ['Height', 'Age', 'From', 'To', 'Nonce', 'Hash', 'Fee', 'Amount'];

    pages.forEach(page => it(`on ${page} includes correct columns`, () => {
        cy.visit(page);
        cy.tableHasOrderedColumns('Transactions', columns);
    }));
});

suite(["@CI"],'blocks table', () => {

    let pages = ['/blocks', '/summary', '/'];
    let columns = ['Height', 'State Hash', 'Slot', 'Age', 'Block Producer', 'Coinbase', 'Transactions', 'SNARKs', 'Coinbase Receiver'];

    pages.forEach(page => it(`on ${page} includes correct columns`, () => {
        cy.visit(page);
        cy.tableHasOrderedColumns('Blocks', columns);
    }));
});

suite(["@CI"],'snarks table', () => {

    let pages = ['/snarks'];
    let columns = ['Height', 'Age', 'Prover', 'Work Ids', 'State Hash', 'Fee'];

    pages.forEach(page => it(`on ${page} includes correct columns`, () => {
        cy.visit(page);
        cy.tableHasOrderedColumns('SNARKs', columns);
    }));
});


suite(["@CI"],'fee transfer table', () => {

    let pages = ['/blocks/3NKyujsdi2GtWA1XC9KJ6nvXeLAd3DNvYrm1PUGEagj9899s1LMz/fee-transfers'];
    let columns = ['Recipient', 'Fee', 'Type', 'Age'];

    pages.forEach(page => it(`on ${page} includes correct columns`, () => {
        cy.visit(page);
        cy.tableHasOrderedColumns('Fee Transfers', columns);
    }));
});