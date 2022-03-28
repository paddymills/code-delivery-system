
import { invoke } from '@tauri-apps/api/tauri-api';

invoke('test')
    .then((message) => console.log(message))
    .catch((error) => console.error(error));
