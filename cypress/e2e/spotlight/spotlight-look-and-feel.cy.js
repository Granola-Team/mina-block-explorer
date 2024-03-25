
const devices = require('../../devices.json');
const { DEFAULT_ACCOUNT_PK } = require('../constants');

suite(["@VisualRegression"],'spotlight', () => {

    const pages = [
        { url: "/summary/accounts/B62qoiQhNWjwFkfCVBpkDTYytifftoYQB9qJ3z6X4d58ocxmiwUQY8U", name: 'account-dialog'},
        { url: `/addresses/accounts/${DEFAULT_ACCOUNT_PK}`, name: 'account-page' },
    ];
    
    pages.forEach(page => {
        devices.forEach(device => {
            it(`has elements correctly positioned on page ${page.name} on ${device}`, () => {
                cy.viewport(device);
                cy.visit(page.url);
                cy.prepareSnapshotTest();
                cy.get('h1').contains('Account Spotlight').should('exist');
                cy.get('.loading-placeholder').should('not.exist');
                cy.get('#spotlight-heading').matchImageSnapshot(`[${page.name}]-[${device}]-spotlight-heading`);
            });
        });
    }); 
});