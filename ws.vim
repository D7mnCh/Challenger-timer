let g:SessionLoad = 1
let s:so_save = &g:so | let s:siso_save = &g:siso | setg so=0 siso=0 | setl so=-1 siso=-1
let v:this_session=expand("<sfile>:p")
doautoall SessionLoadPre
silent only
silent tabonly
cd ~/Projects/random/pomodoro/src
if expand('%') == '' && !&modified && line('$') <= 1 && getline(1) == ''
  let s:wipebuf = bufnr('%')
endif
let s:shortmess_save = &shortmess
set shortmess+=aoO
badd +22 data.rs
badd +19 main.rs
badd +8 intrective_components/pause_button.rs
badd +28 intrective_components.rs
badd +18 static_components/switch_cell.rs
badd +5 intrective_components/turn_off_sound_button.rs
badd +26 intrective_components/work_secs_glider.rs
badd +23 intrective_components/rest_secs_glider.rs
badd +18 static_components/work_cell.rs
badd +1 intrective_components/reset_totals.rs
badd +21 static_components/rest_cell.rs
badd +5 intrective_components/work_button.rs
argglobal
%argdel
$argadd ~/Projects/random/pomodoro
edit intrective_components/work_button.rs
let s:save_splitbelow = &splitbelow
let s:save_splitright = &splitright
set splitbelow splitright
wincmd _ | wincmd |
vsplit
1wincmd h
wincmd _ | wincmd |
split
1wincmd k
wincmd _ | wincmd |
vsplit
1wincmd h
wincmd w
wincmd w
wincmd w
wincmd _ | wincmd |
split
1wincmd k
wincmd w
let &splitbelow = s:save_splitbelow
let &splitright = s:save_splitright
wincmd t
let s:save_winminheight = &winminheight
let s:save_winminwidth = &winminwidth
set winminheight=0
set winheight=1
set winminwidth=0
set winwidth=1
exe '1resize ' . ((&lines * 18 + 19) / 39)
exe 'vert 1resize ' . ((&columns * 58 + 87) / 174)
exe '2resize ' . ((&lines * 18 + 19) / 39)
exe 'vert 2resize ' . ((&columns * 57 + 87) / 174)
exe '3resize ' . ((&lines * 18 + 19) / 39)
exe 'vert 3resize ' . ((&columns * 116 + 87) / 174)
exe '4resize ' . ((&lines * 18 + 19) / 39)
exe 'vert 4resize ' . ((&columns * 57 + 87) / 174)
exe '5resize ' . ((&lines * 18 + 19) / 39)
exe 'vert 5resize ' . ((&columns * 57 + 87) / 174)
argglobal
setlocal foldmethod=manual
setlocal foldexpr=0
setlocal foldmarker={{{,}}}
setlocal foldignore=#
setlocal foldlevel=0
setlocal foldminlines=1
setlocal foldnestmax=20
setlocal foldenable
silent! normal! zE
let &fdl = &fdl
let s:l = 5 - ((4 * winheight(0) + 9) / 18)
if s:l < 1 | let s:l = 1 | endif
keepjumps exe s:l
normal! zt
keepjumps 5
normal! 016|
lcd ~/Projects/random/pomodoro/src
wincmd w
argglobal
if bufexists(fnamemodify("~/Projects/random/pomodoro/src/intrective_components.rs", ":p")) | buffer ~/Projects/random/pomodoro/src/intrective_components.rs | else | edit ~/Projects/random/pomodoro/src/intrective_components.rs | endif
if &buftype ==# 'terminal'
  silent file ~/Projects/random/pomodoro/src/intrective_components.rs
endif
setlocal foldmethod=manual
setlocal foldexpr=0
setlocal foldmarker={{{,}}}
setlocal foldignore=#
setlocal foldlevel=0
setlocal foldminlines=1
setlocal foldnestmax=20
setlocal foldenable
silent! normal! zE
let &fdl = &fdl
let s:l = 28 - ((16 * winheight(0) + 9) / 18)
if s:l < 1 | let s:l = 1 | endif
keepjumps exe s:l
normal! zt
keepjumps 28
normal! 033|
lcd ~/Projects/random/pomodoro/src
wincmd w
argglobal
if bufexists(fnamemodify("~/Projects/random/pomodoro/src/intrective_components/reset_totals.rs", ":p")) | buffer ~/Projects/random/pomodoro/src/intrective_components/reset_totals.rs | else | edit ~/Projects/random/pomodoro/src/intrective_components/reset_totals.rs | endif
if &buftype ==# 'terminal'
  silent file ~/Projects/random/pomodoro/src/intrective_components/reset_totals.rs
endif
setlocal foldmethod=manual
setlocal foldexpr=0
setlocal foldmarker={{{,}}}
setlocal foldignore=#
setlocal foldlevel=0
setlocal foldminlines=1
setlocal foldnestmax=20
setlocal foldenable
silent! normal! zE
let &fdl = &fdl
let s:l = 1 - ((0 * winheight(0) + 9) / 18)
if s:l < 1 | let s:l = 1 | endif
keepjumps exe s:l
normal! zt
keepjumps 1
normal! 0
lcd ~/Projects/random/pomodoro/src
wincmd w
argglobal
if bufexists(fnamemodify("~/Projects/random/pomodoro/src/data.rs", ":p")) | buffer ~/Projects/random/pomodoro/src/data.rs | else | edit ~/Projects/random/pomodoro/src/data.rs | endif
if &buftype ==# 'terminal'
  silent file ~/Projects/random/pomodoro/src/data.rs
endif
setlocal foldmethod=manual
setlocal foldexpr=0
setlocal foldmarker={{{,}}}
setlocal foldignore=#
setlocal foldlevel=0
setlocal foldminlines=1
setlocal foldnestmax=20
setlocal foldenable
silent! normal! zE
let &fdl = &fdl
let s:l = 13 - ((9 * winheight(0) + 9) / 18)
if s:l < 1 | let s:l = 1 | endif
keepjumps exe s:l
normal! zt
keepjumps 13
normal! 023|
lcd ~/Projects/random/pomodoro
wincmd w
argglobal
if bufexists(fnamemodify("~/Projects/random/pomodoro/src/main.rs", ":p")) | buffer ~/Projects/random/pomodoro/src/main.rs | else | edit ~/Projects/random/pomodoro/src/main.rs | endif
if &buftype ==# 'terminal'
  silent file ~/Projects/random/pomodoro/src/main.rs
endif
setlocal foldmethod=manual
setlocal foldexpr=0
setlocal foldmarker={{{,}}}
setlocal foldignore=#
setlocal foldlevel=0
setlocal foldminlines=1
setlocal foldnestmax=20
setlocal foldenable
silent! normal! zE
let &fdl = &fdl
let s:l = 19 - ((8 * winheight(0) + 9) / 18)
if s:l < 1 | let s:l = 1 | endif
keepjumps exe s:l
normal! zt
keepjumps 19
normal! 043|
lcd ~/Projects/random/pomodoro/src
wincmd w
exe '1resize ' . ((&lines * 18 + 19) / 39)
exe 'vert 1resize ' . ((&columns * 58 + 87) / 174)
exe '2resize ' . ((&lines * 18 + 19) / 39)
exe 'vert 2resize ' . ((&columns * 57 + 87) / 174)
exe '3resize ' . ((&lines * 18 + 19) / 39)
exe 'vert 3resize ' . ((&columns * 116 + 87) / 174)
exe '4resize ' . ((&lines * 18 + 19) / 39)
exe 'vert 4resize ' . ((&columns * 57 + 87) / 174)
exe '5resize ' . ((&lines * 18 + 19) / 39)
exe 'vert 5resize ' . ((&columns * 57 + 87) / 174)
tabnext 1
if exists('s:wipebuf') && len(win_findbuf(s:wipebuf)) == 0 && getbufvar(s:wipebuf, '&buftype') isnot# 'terminal'
  silent exe 'bwipe ' . s:wipebuf
endif
unlet! s:wipebuf
set winheight=1 winwidth=20
let &shortmess = s:shortmess_save
let &winminheight = s:save_winminheight
let &winminwidth = s:save_winminwidth
let s:sx = expand("<sfile>:p:r")."x.vim"
if filereadable(s:sx)
  exe "source " . fnameescape(s:sx)
endif
let &g:so = s:so_save | let &g:siso = s:siso_save
set hlsearch
nohlsearch
doautoall SessionLoadPost
unlet g:SessionLoad
" vim: set ft=vim :
