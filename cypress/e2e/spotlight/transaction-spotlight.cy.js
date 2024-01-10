const devices = require('../../devices.json');

describe('transaction spotlight', () => {

    let expected_fields = [
        'Date', 'Payment ID', 'Canonical', 'Amount', 'From', 'Nonce', 'Kind', 
        'Transaction Hash', 'Block Height', 'Block State Hash', 'Fee', 'To', 'Memo'
    ];
    let transaction_id = "CkpYkHBnz3c7mxcKsPP7Y3m69HvFyU5CepLPjxuc1ynTsfQnAkYnT";
    let mobile = devices[0];

    it('displays complete information', () => {
        cy.viewport(mobile);
        cy.visit(`/transactions/${transaction_id}`);
        cy.testSpotlight("Transaction Spotlight", transaction_id, expected_fields);
    });
});