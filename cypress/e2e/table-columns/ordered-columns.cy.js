suite(["@CI"],'transactions table', () => {

    let pages = ['/transactions'];
    let columns = ['Height', 'State Hash', 'Age', 'Type', 'From', 'To', 'Nonce', 'Fee', 'Amount'];

    pages.forEach(page => it(`on ${page} includes correct columns`, () => {
        cy.visit(page);
        cy.tableHasOrderedColumns('Transactions', columns);
    }));
});

suite(["@CI"],'account transactions table', () => {

    let pages = ['/addresses/accounts/B62qrYveCMCW2tr5J8gu9T1rh817zsq7j8cjc9mHEecQS2tRMnoNTsy'];
    let columns = ['Height', 'State Hash', 'Nonce', 'Age','Type', 'Direction', 'Counterparty', 'Amount/Fee'];

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
    let columns = ['Height', 'State Hash', 'Age', 'Prover', 'Work Ids', 'Fee'];

    pages.forEach(page => it(`on ${page} includes correct columns`, () => {
        cy.visit(page);
        cy.tableHasOrderedColumns('SNARKs', columns);
    }));
});


suite(["@CI"],'internal commands table', () => {

    let pages = ['/blocks/3NKyujsdi2GtWA1XC9KJ6nvXeLAd3DNvYrm1PUGEagj9899s1LMz/internal-commands'];
    let columns = ['Recipient', 'Fee', 'Type', 'Age'];

    pages.forEach(page => it(`on ${page} includes correct columns`, () => {
        cy.visit(page);
        cy.tableHasOrderedColumns('Internal Commands', columns);
    }));
});