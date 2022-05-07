const wallet = {
    web3: null,
    chainID: null,
    userAddress: null,
    BN: null,
    setup: async function () {
        try{
            if (window.ethereum) {
                this.web3 = window.ethereum;
                console.log("web3 ethereum");
            }
            else if (window.web3) {
                this.web3 = window.web3;
                console.log('Injected web3 detected.');
            }
            else {
                console.error("Can't find web3 wallet");
                return;
            }
            // this.BN = this.web3.utils.BN;
            await this.web3.enable();
            this.chainID = await this.web3.networkVersion;
            this.userAddress = await this.web3.selectedAddress;
            this.reload_on_change();
        }catch(e){
            console.error(e);
        }
    },
    get_user_address: async function (){
        if(await this.is_ready()){
            if (this.userAddress!==null) return this.userAddress;

            return await await this.web3.selectedAddress;
        }
    },
    is_ready: async function (){
        if(this.web3!==null && this.userAddress!==null) return true;
        return new Promise(r=>{
            if(this.web3!==null && this.userAddress!==null) r(true);
            let t = 10;
            let i = setInterval(() => {
                t-=1;
                if(this.web3!==null && this.userAddress!==null) {
                    clearInterval(i); 
                    r(true);
                };
                if(t<=0) {
                    console.error("Error connecting to the wallet", "danger", 5000); 
                    clearInterval(i);
                    r(false);
                }
            }, 1000);
        })
    },
    is_correct_chain: async function (id){
        if(await this.is_ready()){
            if (parseInt(this.chainID)!==parseInt(id)){
                console.error(`Wallet is connected to "${chain_id_to_name(this.chainID)}" instead of "${chain_id_to_name(id)}"`, "warning", null);
            }
            return parseInt(this.chainID)===parseInt(id);
        }
        return false
    },
    get_chain: async function (){
        if(await this.is_ready()){
            if (parseInt(this.chainID)!==null){
                return this.chainID;
            }
            return await this.web3.eth.networkVersion;
        }
        return null
    },
    reload_on_change: function () {
        // console.log(web3)
        setInterval(async ()=>{
            if (this.web3 !== null) {
                let cur_address = this.userAddress!==null? this.userAddress: "";
                let cur_chainID = this.chainID!==null? this.chainID: -1;
                try {
                    // let netName = await this.web3.eth.net.getNetworkType();
                    this.chainID = await this.web3.networkVersion;
                    this.userAddress = await this.web3.selectedAddress;
                    if (cur_address && cur_address !== this.userAddress) window.location.reload(false);
                    if (cur_chainID && cur_chainID !== this.chainID) window.location.reload(false);
                } catch (error) { console.error(error); this.web3 = null; }
            } else {
                // alert("Can't find web3 wallet!")
            }
        }, 1000);
    },

    // setWalletDisconnected: function () {
    //     window.location.reload();
    // }
}