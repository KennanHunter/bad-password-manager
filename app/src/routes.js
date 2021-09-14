import Vault from "./Routes/Vault.svelte";
import Login from "./Routes/Login.svelte";
import Lost from "./Routes/Lost.svelte";

export default {
	"/": Login,
	"/vault/": Vault,
	"*": Lost,
};
