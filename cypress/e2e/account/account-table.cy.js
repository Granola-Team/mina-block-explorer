import { DEFAULT_ACCOUNT_PK } from "../constants";

suite(["@CI"],'account transaction table',()=>{

    it('correctly references a counterparty', () => {
        cy.visit(`/addresses/accounts/${account}`);
        cy.aliasTableValue(0,'Counterparty','Transactions', 'counterparty');

        for(let i=0;i<=9;i++)
        {
            cy.aliasTableValue(i,'Counterparty','Transactions', 'counterparty');
            cy.get('@counterparty').invoke('text').then(text => {
                if (text == "Self") {
                    expect(true).to.equal(true);
                } else {
                    expect(text.length).to.equal(DEFAULT_ACCOUNT_PK.length);
                }
                expect(text).to.not.equal(DEFAULT_ACCOUNT_PK);
            })
            
        }
        
    });
    
})