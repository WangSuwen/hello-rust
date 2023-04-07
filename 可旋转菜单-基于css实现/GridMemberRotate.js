// 网格员转盘组件 -- 基于 Vue2.0 + vue-styled-components
import styled from 'vue-styled-components';
import Vue from 'vue';
const GMContainer = styled('div', {
	rotate: {
		type: Number,
		default: 0
	},
	containerRX: {
		type: Number,
		default: 0
	},
	avatarBg: {
		type: String
	}
})`
	// width: 9.02rem;
	width: 100%;
	height: 6.52rem;
	position: relative;
	& .bg-1 {
		position: absolute;
		transition: all 0.3s;
		top: -0.7rem;
		width: 9.02rem;
		height: 9.02rem;
		// transform: rotateX(72.3deg) rotate(${props => props.rotate}deg);
		animation: spin 6s linear infinite;
		@keyframes spin {
			from {
				transform: rotateX(72.3deg) rotate(0deg);
			}
			to {
				transform: rotateX(72.3deg) rotate(360deg);
			}
		}
	}
	& .bg-2 {
		position: absolute;
		// width: 9.28rem;
		width: 100%;
		height: 4.94rem;
		top: 1.62rem;
		left: -0.02rem;
		transform: rotateX(50deg);
		z-index: -1;
	}
	& .bg-3 {
		position: absolute;
		width: 5.28rem;
		height: 2.94rem;
		top: 98px;
		left: 80px;
		transform: rotateX(50deg);
		z-index: -1;
	}
	& .bg-4 {
		position: absolute;
		width: 3.2rem;
		height: 6.94rem;
		top: -57px;
		left: 125px;
		transform: rotateX(50deg);
		z-index: -1;
	}
	& .rb {
		position: absolute;
		cursor: pointer;
		&.right {
			left: 4rem;
		}
	}
	& .stage_area {
		position: absolute;
		top: 0;
		left: 0;
		width: 100%;
		height: 100%;
		perspective: 857px;
    	perspective-origin: top;
		& .container {
            width: 128px;
            height: 100px;
            margin-left: -64px;
            transform-style: preserve-3d;
            position: absolute;
            top: 19%;
            left: 50%;
			cursor: pointer;
			&.container-ts {
				transition: all ease 1s;
			}
			&.animate {
				animation: move 18s infinite linear;
			}
			@keyframes move {
				0% {
					transform: rotateX(${props => props.containerRX}deg) rotateY(0deg);
				}
				100% {
					transform: rotateX(${props => props.containerRX}deg) rotateY(-360deg);
				}
			}
			& .piece {
				position: absolute;
				bottom: 0;
				border-radius: 50%;
				width: 1.8rem;
				height: 1.8rem;
				border: 0.047rem solid rgba(0, 114, 255, 0.80);
				&.normal-bg {
					background-color: rgba(16, 108, 222, 0.22);
				}
				&.active-bg {
					background: url(${props => props.avatarBg}) no-repeat center;
					background-size: contain;
				}
				& .user-avatar {
					width: 1.6rem;
					height: 1.6rem;
					border-radius: 50%;
				}
				& .userinfo-bottom {
					position: absolute;
					z-index: -1;
					bottom: -0.83rem;
					width: 2.9rem;
					height: 1.3rem;
				}
			}
        }
	}
`;
export default Vue.component('GridMemberRotate', {
	props: {
		userList: {
			type: Array,
			default: () => ([])
		}
	},
	data () {
		return {
			rotateStep: 60,
			rotateValue: 0,
			rotate: 0,
			activeInd: 0,
			indexPiece: 0,
			stopAnimation: false,
			stopTransition: false,
			containerRX: 353,
			containerTF: ''
		};
	},
	created () {
		this.rotate = 360 / this.userList.length;
	},
	methods: {
		// 鼠标是否移入
		mouseEnter (type) {
			if (type !== this.stopTransition) {
				this.stopTransition = type;
				if (type) {
					this.containerTF = `rotateX(${this.containerRX}deg) rotateY(0deg)`;
					this.$emit('userChange', this.userList[this.activeInd]);
				}
				!type && (this.indexPiece = 0, this.activeInd = 0);
				setTimeout(() => {
					this.stopAnimation = type;
				}, 300);
			}
		},
		mouseClick () {
			if (this.activeInd < this.userList.length - 1) {
				this.activeInd++;
			} else {
				this.activeInd = 0;
			}
			this.$emit('userChange', this.userList[this.activeInd]);
			this.containerTF = `rotateX(${this.containerRX}deg) rotateY(${-1 * this.rotate * ++this.indexPiece}deg)`;
		}
	},
	render: function (h) {
		const avabg = require('@/assets/images/brain/city-operation/userinfo-bg.png');
		// TODO: 这里需要判 0 ；
		const transZ = 64 / Math.tan(1 / this.userList.length * Math.PI);
		return (
			<GMContainer
				rotate={this.rotateValue}
				avatarBg={avabg}
				containerRX={this.containerRX}
			>
				{
					[1, 2, 3, 4].map(item => (
						<img class={`bg-${item}`} src={require(`@/assets/images/brain/city-operation/gm-bg${item}.png`)}/>
					))
				}
				<div
					class='stage_area flex f-ai-c f-jc-c'
					onMouseover={() => this.mouseEnter(true)}
					onMouseleave={() => this.mouseEnter(false)}
					onClick={this.mouseClick }
				>
            		<div
						id='container'
						class={`container flex f-ai-c f-jc-c ${this.stopAnimation ? '' : 'animate'} ${this.stopTransition ? 'container-ts' : ''}`}
						style={`
							transform: ${this.containerTF}
						`}
					>
						{
							this.userList.map((item, index) => (
								<div
									class={`piece flex f-ai-c f-jc-c ${this.activeInd === index ? 'active-bg' : 'normal-bg'}`}
									style={
										`transform: rotateY(${index * this.rotate}deg) translateZ(${transZ + 60}px)`
									}
								>
									<img
										src={item.img}
										class='user-avatar'
									/>
									<img
										src={require('@/assets/images/brain/city-operation/userinfo-bottom.png')}
										class='userinfo-bottom'
									/>
								</div>
							))
						}
					</div>
				</div>
			</GMContainer>
		);
	}
});
