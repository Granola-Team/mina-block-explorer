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
})