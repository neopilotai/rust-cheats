if exists('g:loaded_rust_cheats')
  finish
endif
let g:loaded_rust_cheats = 1

function! s:RustCheats(query) abort
  if empty(a:query)
    let l:query = input('Cheat-sheet search: ')
  else
    let l:query = a:query
  endif
  if empty(l:query) | return | endif
  execute 'new'
  setlocal buftype=nofile bufhidden=wipe noswapfile
  execute 'file [rust-cheats]'
  let l:output = systemlist('rust-cheats search ' . shellescape(l:query))
  call setline(1, empty(l:output) ? ['No results.'] : l:output)
  setlocal nomodifiable
endfunction

command! -nargs=* RustCheats call <SID>RustCheats(<q-args>)
