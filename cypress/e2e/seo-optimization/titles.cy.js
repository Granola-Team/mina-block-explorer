suite(["@CI"],'meta title',() => {

    let public_key="B62qpge4uMq4Vv5Rvc8Gw9qSquUYd6xoW1pz7HQkMSHm6h1o7pvLPAN";
    let pages = [
        { url: `/addresses/accounts`, title: `Accounts | Search For Mina Account`},
        { url: `/addresses/accounts/${public_key}`, title: `Account Overview | 'MinaExplorer'`},
        { url: '/blocks', title: 'Blocks | Search for Mina Blocks'},
    ];

    pages.forEach(({title, url}) => it(`'${title}' exists in <head> for page ${url}`, () => {
        cy.visit(url);
        cy.title({timeout: 20000}).should('equal', title);
    }));

})
