call plug#begin('~/.vim/plugged')

Plug 'vim-airline/vim-airline'
Plug 'gerardbm/vim-airline-themes'
Plug 'neoclide/coc.nvim', {'branch': 'release'}
Plug 'preservim/nerdcommenter', { 'commit': 'a5d1663' }
Plug 'preservim/nerdtree'
Plug 'valloric/listtoggle'
Plug 'Shougo/neosnippet.vim', { 'commit': '037b7a7' }
Plug 'Shougo/neosnippet-snippets'
Plug 'Shougo/context_filetype.vim', { 'commit': 'e276626' }
Plug 'rust-lang/rust.vim'
Plug 'junegunn/fzf', { 'do': { -> fzf#install() } }
Plug 'junegunn/fzf.vim'
Plug 'ctrlpvim/ctrlp.vim'
Plug 'psliwka/vim-smoothie'
Plug 'camspiers/animate.vim'
Plug 'camspiers/lens.vim'

call plug#end()

set nocompatible
syntax enable
filetype plugin indent on
set number
set relativenumber
set cursorline
set ruler
set scrolloff=5

augroup numbertoggle
  autocmd!
  autocmd BufEnter,FocusGained,InsertLeave * set relativenumber
  autocmd BufLeave,FocusLost,InsertEnter   * set norelativenumber
augroup END

inoremap <silent><expr> <C-Space> coc#refresh()
inoremap <silent><expr> <Tab> coc#pum#visible() ? coc#pum#confirm() : "\<Tab>"
inoremap <silent><expr> <S-Tab> coc#pum#visible() ? coc#pum#select_prev() : "\<C-h>"
autocmd CursorHold * silent call CocActionAsync('highlight')

nnoremap <F2> :NERDTreeToggle<CR>
nnoremap <F3> :vert terminal<CR>
nnoremap <leader>z :Goyo<CR>
nmap s <Plug>(easymotion-s2)
nnoremap <leader>f :Files<CR>
nnoremap <leader>b :Buffers<CR>
nnoremap <leader>g :GFiles<CR>

set hlsearch
set incsearch
set wrapscan
set ignorecase
set smartcase
set magic
set maxmempattern=1000
let g:rustfmt_autosave = 1

let g:NERDDefaultAlign          = 'left'
let g:NERDSpaceDelims           = 1
let g:NERDCompactSexyComs       = 1
let g:NERDCommentEmptyLines     = 0
let g:NERDCreateDefaultMappings = 0
let g:NERDCustomDelimiters      = {
	\ 'python': {'left': '#'},
	\ }
nnoremap cc :call NERDComment(0,'toggle')<CR>
vnoremap cc :call NERDComment(0,'toggle')<CR>

let g:ale_sign_error='✗'
let g:ale_sign_warning='•'
let g:ale_sign_info='ℹ'

nnoremap <Space> /
nnoremap <Leader><Space> ?

nnoremap <silent> <Leader><CR> :let @/='\<<C-R>=expand("<cword>")<CR>\>'<CR>:set hlsearch<CR>
vnoremap <silent> <Leader><CR> :<C-U>call <SID>VSetSearch()<CR>:set hlsearch<CR>
nnoremap <Leader>m :noh<CR>
nnoremap n nzz
nnoremap N Nzz

vnoremap <silent> <Space> :<C-U>call <SID>RangeSearch('/')<CR>
	\ :if strlen(g:srchstr) > 0
	\ \|exec '/'.g:srchstr\|endif<CR>n
vnoremap <silent> ? :<C-U>call <SID>RangeSearch('?')<CR>
	\ :if strlen(g:srchstr) > 0
	\ \|exec '?'.g:srchstr\|endif<CR>N
